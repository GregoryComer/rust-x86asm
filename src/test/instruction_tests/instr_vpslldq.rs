use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 253, 1], OperandSize::Dword)
}

#[test]
fn vpslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 254, 111], OperandSize::Qword)
}

#[test]
fn vpslldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 115, 254, 27], OperandSize::Dword)
}

#[test]
fn vpslldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 253, 22], OperandSize::Qword)
}

#[test]
fn vpslldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 115, 249, 52], OperandSize::Dword)
}

#[test]
fn vpslldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 8, 115, 60, 254, 3], OperandSize::Dword)
}

#[test]
fn vpslldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 101, 8, 115, 255, 47], OperandSize::Qword)
}

#[test]
fn vpslldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 53, 8, 115, 56, 102], OperandSize::Qword)
}

#[test]
fn vpslldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 115, 252, 53], OperandSize::Dword)
}

#[test]
fn vpslldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 115, 60, 247, 66], OperandSize::Dword)
}

#[test]
fn vpslldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM28)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 21, 32, 115, 252, 58], OperandSize::Qword)
}

#[test]
fn vpslldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 32, 115, 60, 75, 63], OperandSize::Qword)
}

#[test]
fn vpslldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 249, 29], OperandSize::Dword)
}

#[test]
fn vpslldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1229979594, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 72, 115, 188, 185, 202, 255, 79, 73, 27], OperandSize::Dword)
}

#[test]
fn vpslldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 5, 64, 115, 250, 63], OperandSize::Qword)
}

#[test]
fn vpslldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectDisplaced(RCX, 850091521, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 45, 72, 115, 185, 1, 94, 171, 50, 19], OperandSize::Qword)
}

