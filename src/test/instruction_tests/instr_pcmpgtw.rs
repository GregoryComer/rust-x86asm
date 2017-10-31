use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 243], OperandSize::Dword)
}

#[test]
fn pcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1321103265, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 187, 161, 111, 190, 78], OperandSize::Dword)
}

#[test]
fn pcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 238], OperandSize::Qword)
}

#[test]
fn pcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RBX, 2129516161, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 131, 129, 214, 237, 126], OperandSize::Qword)
}

#[test]
fn pcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 238], OperandSize::Dword)
}

#[test]
fn pcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1387081068, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 148, 90, 108, 45, 173, 82], OperandSize::Dword)
}

#[test]
fn pcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 236], OperandSize::Qword)
}

#[test]
fn pcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1343318406, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 164, 131, 134, 105, 17, 80], OperandSize::Qword)
}

