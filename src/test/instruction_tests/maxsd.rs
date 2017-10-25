use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn maxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 249], OperandSize::Dword)
}

fn maxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1652806653, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 44, 221, 253, 211, 131, 98], OperandSize::Dword)
}

fn maxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 241], OperandSize::Qword)
}

fn maxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1852392022, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 20, 205, 86, 66, 105, 110], OperandSize::Qword)
}

