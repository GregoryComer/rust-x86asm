use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 222, 44], OperandSize::Dword)
}

#[test]
fn vpsrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 115, 217, 62], OperandSize::Qword)
}

#[test]
fn vpsrldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 219, 60], OperandSize::Dword)
}

#[test]
fn vpsrldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 115, 223, 89], OperandSize::Qword)
}

#[test]
fn vpsrldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 218, 105], OperandSize::Dword)
}

#[test]
fn vpsrldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 835075344, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 8, 115, 156, 86, 16, 61, 198, 49, 110], OperandSize::Dword)
}

#[test]
fn vpsrldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM17)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 53, 8, 115, 217, 40], OperandSize::Qword)
}

#[test]
fn vpsrldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 681780529, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 0, 115, 28, 149, 49, 37, 163, 40, 26], OperandSize::Qword)
}

#[test]
fn vpsrldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 223, 53], OperandSize::Dword)
}

#[test]
fn vpsrldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1085957988, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 115, 156, 139, 100, 103, 186, 64, 100], OperandSize::Dword)
}

#[test]
fn vpsrldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 21, 32, 115, 218, 52], OperandSize::Qword)
}

#[test]
fn vpsrldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 45, 32, 115, 30, 35], OperandSize::Qword)
}

#[test]
fn vpsrldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 115, 223, 123], OperandSize::Dword)
}

#[test]
fn vpsrldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 496895416, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 72, 115, 28, 141, 184, 5, 158, 29, 116], OperandSize::Dword)
}

#[test]
fn vpsrldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 117, 72, 115, 217, 44], OperandSize::Qword)
}

#[test]
fn vpsrldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 260769911, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 72, 115, 28, 125, 119, 8, 139, 15, 124], OperandSize::Qword)
}

