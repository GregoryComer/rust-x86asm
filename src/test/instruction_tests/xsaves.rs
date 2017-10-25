use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsaves_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectDisplaced(DI, 105, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 109, 105], OperandSize::Word)
}

fn xsaves_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 44, 120], OperandSize::Dword)
}

fn xsaves_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 44, 191], OperandSize::Qword)
}

