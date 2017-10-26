use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 210, 14], OperandSize::Dword)
}

#[test]
fn roundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 552635162, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 151, 26, 139, 240, 32, 39], OperandSize::Dword)
}

#[test]
fn roundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 244, 1], OperandSize::Qword)
}

#[test]
fn roundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1215021873, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 188, 190, 49, 195, 107, 72, 26], OperandSize::Qword)
}

