use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 252, 41], OperandSize::Dword)
}

#[test]
fn cmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 331678156, Some(OperandSize::Qword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 180, 118, 204, 1, 197, 19, 127], OperandSize::Dword)
}

#[test]
fn cmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 195, 119], OperandSize::Qword)
}

#[test]
fn cmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 44, 71, 88], OperandSize::Qword)
}

