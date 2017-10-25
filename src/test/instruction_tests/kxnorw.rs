use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxnorw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORW, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 70, 203], OperandSize::Dword)
}

fn kxnorw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORW, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 70, 230], OperandSize::Qword)
}

