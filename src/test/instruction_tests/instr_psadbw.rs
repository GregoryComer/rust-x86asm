use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 224], OperandSize::Dword)
}

#[test]
fn psadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1434102233, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 60, 77, 217, 169, 122, 85], OperandSize::Dword)
}

#[test]
fn psadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 225], OperandSize::Qword)
}

#[test]
fn psadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 63], OperandSize::Qword)
}

#[test]
fn psadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 195], OperandSize::Dword)
}

#[test]
fn psadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 420241611, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 4, 213, 203, 96, 12, 25], OperandSize::Dword)
}

#[test]
fn psadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 236], OperandSize::Qword)
}

#[test]
fn psadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 52, 211], OperandSize::Qword)
}

