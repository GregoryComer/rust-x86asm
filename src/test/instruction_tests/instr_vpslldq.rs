use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 254, 71], OperandSize::Dword)
}

#[test]
fn vpslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 253, 50], OperandSize::Qword)
}

#[test]
fn vpslldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 255, 52], OperandSize::Dword)
}

#[test]
fn vpslldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 115, 253, 126], OperandSize::Qword)
}

#[test]
fn vpslldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 252, 21], OperandSize::Dword)
}

#[test]
fn vpslldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 915716151, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 8, 115, 188, 91, 55, 184, 148, 54, 85], OperandSize::Dword)
}

#[test]
fn vpslldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 193, 25, 115, 251, 45], OperandSize::Qword)
}

#[test]
fn vpslldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RBX, 2125095638, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 53, 0, 115, 187, 214, 98, 170, 126, 13], OperandSize::Qword)
}

#[test]
fn vpslldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 253, 69], OperandSize::Dword)
}

#[test]
fn vpslldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 633830606, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 115, 60, 205, 206, 124, 199, 37, 70], OperandSize::Dword)
}

#[test]
fn vpslldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 13, 32, 115, 248, 77], OperandSize::Qword)
}

#[test]
fn vpslldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1791414256, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 32, 115, 60, 125, 240, 207, 198, 106, 67], OperandSize::Qword)
}

#[test]
fn vpslldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 109, 72, 115, 248, 21], OperandSize::Dword)
}

#[test]
fn vpslldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 109, 72, 115, 56, 59], OperandSize::Dword)
}

#[test]
fn vpslldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 53, 64, 115, 249, 102], OperandSize::Qword)
}

#[test]
fn vpslldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 728257329, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 61, 72, 115, 188, 127, 49, 83, 104, 43, 24], OperandSize::Qword)
}

