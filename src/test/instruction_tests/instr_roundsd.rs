use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 195, 81], OperandSize::Dword)
}

#[test]
fn roundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 44, 177, 101], OperandSize::Dword)
}

#[test]
fn roundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 251, 68], OperandSize::Qword)
}

#[test]
fn roundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 1504108805, Some(OperandSize::Qword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 143, 5, 225, 166, 89, 50], OperandSize::Qword)
}

