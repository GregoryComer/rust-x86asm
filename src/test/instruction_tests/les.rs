use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn les_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(CX)), operand2: Some(Indirect(SI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 12], OperandSize::Word)
}

fn les_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 364033544, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 196, 44, 77, 8, 182, 178, 21], OperandSize::Dword)
}

fn les_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1495242058, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 188, 251, 74, 149, 31, 89], OperandSize::Dword)
}

