use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn insertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 192, 17], OperandSize::Dword)
}

fn insertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 316833872, Some(OperandSize::Dword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 140, 126, 80, 128, 226, 18, 47], OperandSize::Dword)
}

fn insertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 208, 124], OperandSize::Qword)
}

fn insertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1722026810, Some(OperandSize::Dword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 20, 77, 58, 11, 164, 102, 60], OperandSize::Qword)
}

