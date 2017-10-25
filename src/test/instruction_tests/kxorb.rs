use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxorb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORB, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 71, 227], OperandSize::Dword)
}

fn kxorb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORB, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 71, 207], OperandSize::Qword)
}

