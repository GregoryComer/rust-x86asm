use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K3)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 228, 71, 225], OperandSize::Dword)
}

fn kxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K2)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 236, 71, 242], OperandSize::Qword)
}

