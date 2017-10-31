use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 209, 67], OperandSize::Dword)
}

#[test]
fn psrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM6)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 214, 119], OperandSize::Qword)
}

#[test]
fn psrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 215, 63], OperandSize::Dword)
}

#[test]
fn psrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 209, 85], OperandSize::Qword)
}

#[test]
fn psrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 244], OperandSize::Dword)
}

#[test]
fn psrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 2132472499, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 188, 199, 179, 242, 26, 127], OperandSize::Dword)
}

#[test]
fn psrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 222], OperandSize::Qword)
}

#[test]
fn psrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 28, 241], OperandSize::Qword)
}

#[test]
fn psrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 233], OperandSize::Dword)
}

#[test]
fn psrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 918261903, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 12, 93, 143, 144, 187, 54], OperandSize::Dword)
}

#[test]
fn psrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 203], OperandSize::Qword)
}

#[test]
fn psrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 402605997, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 158, 173, 71, 255, 23], OperandSize::Qword)
}

