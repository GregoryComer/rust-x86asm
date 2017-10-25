use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 221, 29], OperandSize::Dword)
}

#[test]
fn vpsrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 221, 121], OperandSize::Qword)
}

#[test]
fn vpsrldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 220, 95], OperandSize::Dword)
}

#[test]
fn vpsrldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 222, 107], OperandSize::Qword)
}

#[test]
fn vpsrldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 218, 11], OperandSize::Dword)
}

#[test]
fn vpsrldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1040572661, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 8, 115, 156, 80, 245, 224, 5, 62, 4], OperandSize::Dword)
}

#[test]
fn vpsrldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 0, 115, 217, 5], OperandSize::Qword)
}

#[test]
fn vpsrldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM12)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 29, 8, 115, 25, 47], OperandSize::Qword)
}

#[test]
fn vpsrldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 115, 217, 85], OperandSize::Dword)
}

#[test]
fn vpsrldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ECX, 1220537697, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 115, 153, 97, 237, 191, 72, 6], OperandSize::Dword)
}

#[test]
fn vpsrldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 69, 32, 115, 221, 78], OperandSize::Qword)
}

#[test]
fn vpsrldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1048909020, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 53, 40, 115, 156, 80, 220, 20, 133, 62, 19], OperandSize::Qword)
}

#[test]
fn vpsrldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 72, 115, 222, 33], OperandSize::Dword)
}

#[test]
fn vpsrldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1047465349, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 72, 115, 28, 69, 133, 13, 111, 62, 50], OperandSize::Dword)
}

#[test]
fn vpsrldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 85, 72, 115, 219, 124], OperandSize::Qword)
}

#[test]
fn vpsrldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 5, 64, 115, 28, 192, 72], OperandSize::Qword)
}

