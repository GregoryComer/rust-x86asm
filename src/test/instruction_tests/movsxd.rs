use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movsxd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 219], OperandSize::Qword)
}

fn movsxd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1101550921, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 180, 129, 73, 85, 168, 65], OperandSize::Qword)
}

