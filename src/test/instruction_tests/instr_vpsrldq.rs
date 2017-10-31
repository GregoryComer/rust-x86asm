use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 216, 10], OperandSize::Dword)
}

#[test]
fn vpsrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 217, 61], OperandSize::Qword)
}

#[test]
fn vpsrldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 219, 52], OperandSize::Dword)
}

#[test]
fn vpsrldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 223, 30], OperandSize::Qword)
}

#[test]
fn vpsrldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 115, 223, 54], OperandSize::Dword)
}

#[test]
fn vpsrldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 206970056, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 8, 115, 156, 75, 200, 28, 86, 12, 36], OperandSize::Dword)
}

#[test]
fn vpsrldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 13, 0, 115, 216, 108], OperandSize::Qword)
}

#[test]
fn vpsrldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 61, 0, 115, 26, 88], OperandSize::Qword)
}

#[test]
fn vpsrldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 115, 219, 7], OperandSize::Dword)
}

#[test]
fn vpsrldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 353748749, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 40, 115, 158, 13, 199, 21, 21, 115], OperandSize::Dword)
}

#[test]
fn vpsrldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM26)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 101, 40, 115, 218, 85], OperandSize::Qword)
}

#[test]
fn vpsrldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RBX, 1014667714, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 29, 40, 115, 155, 194, 153, 122, 60, 115], OperandSize::Qword)
}

#[test]
fn vpsrldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 72, 115, 220, 89], OperandSize::Dword)
}

#[test]
fn vpsrldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 835839263, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 156, 150, 31, 229, 209, 49, 36], OperandSize::Dword)
}

#[test]
fn vpsrldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM20)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 109, 72, 115, 220, 119], OperandSize::Qword)
}

#[test]
fn vpsrldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1488938831, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 13, 64, 115, 156, 89, 79, 103, 191, 88, 7], OperandSize::Qword)
}

