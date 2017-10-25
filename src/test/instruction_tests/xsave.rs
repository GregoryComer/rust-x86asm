use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 22395, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 163, 123, 87], OperandSize::Word)
}

fn xsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1640861739, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 164, 159, 43, 144, 205, 97], OperandSize::Dword)
}

fn xsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectDisplaced(RDX, 744713054, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 162, 94, 107, 99, 44], OperandSize::Qword)
}

