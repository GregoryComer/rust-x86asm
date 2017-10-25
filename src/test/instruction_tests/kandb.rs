use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDB, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 65, 244], OperandSize::Dword)
}

fn kandb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDB, operand1: Some(Direct(K7)), operand2: Some(Direct(K1)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 65, 253], OperandSize::Qword)
}

