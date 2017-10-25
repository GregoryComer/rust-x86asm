use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K1)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 233, 54], OperandSize::Dword)
}

fn kshiftrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K3)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 227, 2], OperandSize::Qword)
}

