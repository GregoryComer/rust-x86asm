use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 231, 92], OperandSize::Dword)
}

#[test]
fn cmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1860346812, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 148, 223, 188, 163, 226, 110, 87], OperandSize::Dword)
}

#[test]
fn cmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 222, 15], OperandSize::Qword)
}

#[test]
fn cmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1761926542, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 132, 82, 142, 221, 4, 105, 36], OperandSize::Qword)
}

