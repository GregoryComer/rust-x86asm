use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invpcid_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 52531761, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 36, 181, 49, 146, 33, 3], OperandSize::Dword)
}

#[test]
fn invpcid_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RCX, 770403077, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 153, 5, 107, 235, 45], OperandSize::Qword)
}

