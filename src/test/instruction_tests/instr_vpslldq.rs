use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 252, 56], OperandSize::Dword)
}

#[test]
fn vpslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 252, 62], OperandSize::Qword)
}

#[test]
fn vpslldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 253, 105], OperandSize::Dword)
}

#[test]
fn vpslldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 248, 50], OperandSize::Qword)
}

#[test]
fn vpslldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 115, 253, 70], OperandSize::Dword)
}

#[test]
fn vpslldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1923814656, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 8, 115, 60, 69, 0, 21, 171, 114, 53], OperandSize::Dword)
}

#[test]
fn vpslldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 117, 0, 115, 253, 120], OperandSize::Qword)
}

#[test]
fn vpslldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM15)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 5, 8, 115, 63, 57], OperandSize::Qword)
}

#[test]
fn vpslldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 250, 96], OperandSize::Dword)
}

#[test]
fn vpslldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 1306984804, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 40, 115, 185, 100, 1, 231, 77, 18], OperandSize::Dword)
}

#[test]
fn vpslldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 53, 32, 115, 249, 42], OperandSize::Qword)
}

#[test]
fn vpslldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1288148411, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 45, 32, 115, 60, 189, 187, 149, 199, 76, 40], OperandSize::Qword)
}

#[test]
fn vpslldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 72, 115, 248, 112], OperandSize::Dword)
}

#[test]
fn vpslldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1517592615, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 72, 115, 188, 137, 39, 160, 116, 90, 29], OperandSize::Dword)
}

#[test]
fn vpslldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 72, 115, 253, 111], OperandSize::Qword)
}

#[test]
fn vpslldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 21, 72, 115, 60, 243, 97], OperandSize::Qword)
}

