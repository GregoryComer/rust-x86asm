use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 232], OperandSize::Dword)
}

#[test]
fn comisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 13392794, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 36, 221, 154, 91, 204, 0], OperandSize::Dword)
}

#[test]
fn comisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 221], OperandSize::Qword)
}

#[test]
fn comisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 60, 186], OperandSize::Qword)
}

