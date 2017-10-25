use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndcu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND0)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 193], OperandSize::Dword)
}

fn bndcu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 335189884, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 148, 206, 124, 151, 250, 19], OperandSize::Dword)
}

fn bndcu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND1)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 205], OperandSize::Qword)
}

fn bndcu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND1)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 11], OperandSize::Qword)
}

