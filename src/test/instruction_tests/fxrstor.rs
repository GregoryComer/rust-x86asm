use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fxrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 13], OperandSize::Word)
}

fn fxrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1303234622, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 140, 87, 62, 200, 173, 77], OperandSize::Dword)
}

fn fxrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 468169493, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 140, 251, 21, 179, 231, 27], OperandSize::Qword)
}

