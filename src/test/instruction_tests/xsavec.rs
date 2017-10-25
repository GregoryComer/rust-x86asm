use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsavec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 105, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 99, 105], OperandSize::Word)
}

fn xsavec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 36, 130], OperandSize::Dword)
}

fn xsavec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(RDI, 1273549603, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 167, 35, 211, 232, 75], OperandSize::Qword)
}

