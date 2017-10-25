use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 220], OperandSize::Dword)
}

#[test]
fn psadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 57], OperandSize::Dword)
}

#[test]
fn psadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 210], OperandSize::Qword)
}

#[test]
fn psadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 44, 208], OperandSize::Qword)
}

#[test]
fn psadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 233], OperandSize::Dword)
}

#[test]
fn psadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 47], OperandSize::Dword)
}

#[test]
fn psadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 194], OperandSize::Qword)
}

#[test]
fn psadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 15], OperandSize::Qword)
}

