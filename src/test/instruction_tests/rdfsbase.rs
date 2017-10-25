use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rdfsbase_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RDFSBASE, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 174, 194], OperandSize::Qword)
}

fn rdfsbase_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RDFSBASE, operand1: Some(Direct(RDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 174, 199], OperandSize::Qword)
}

