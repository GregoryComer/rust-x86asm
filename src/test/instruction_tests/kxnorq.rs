use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxnorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K3)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 228, 70, 255], OperandSize::Dword)
}

fn kxnorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 204, 70, 234], OperandSize::Qword)
}

