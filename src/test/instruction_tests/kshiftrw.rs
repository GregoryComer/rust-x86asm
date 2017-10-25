use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 229, 110], OperandSize::Dword)
}

fn kshiftrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K2)), operand2: Some(Direct(K4)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 212, 25], OperandSize::Qword)
}

