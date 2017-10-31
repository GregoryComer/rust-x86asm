use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 250, 7], OperandSize::Dword)
}

#[test]
fn vpslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 115, 252, 67], OperandSize::Qword)
}

#[test]
fn vpslldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 251, 120], OperandSize::Dword)
}

#[test]
fn vpslldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 250, 77], OperandSize::Qword)
}

#[test]
fn vpslldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 248, 62], OperandSize::Dword)
}

#[test]
fn vpslldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 8, 115, 58, 49], OperandSize::Dword)
}

#[test]
fn vpslldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 77, 0, 115, 254, 24], OperandSize::Qword)
}

#[test]
fn vpslldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM19)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 0, 115, 59, 51], OperandSize::Qword)
}

#[test]
fn vpslldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 253, 55], OperandSize::Dword)
}

#[test]
fn vpslldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 115, 60, 131, 41], OperandSize::Dword)
}

#[test]
fn vpslldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM31)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 37, 32, 115, 255, 34], OperandSize::Qword)
}

#[test]
fn vpslldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM31)), operand2: Some(IndirectDisplaced(RAX, 11005250, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 5, 32, 115, 184, 66, 237, 167, 0, 6], OperandSize::Qword)
}

#[test]
fn vpslldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 72, 115, 248, 118], OperandSize::Dword)
}

#[test]
fn vpslldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 60, 90, 0], OperandSize::Dword)
}

#[test]
fn vpslldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 72, 115, 249, 17], OperandSize::Qword)
}

#[test]
fn vpslldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RBX, 76171341, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 64, 115, 187, 77, 72, 138, 4, 68], OperandSize::Qword)
}

