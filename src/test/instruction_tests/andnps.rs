use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn andnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 234], OperandSize::Dword)
}

fn andnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 72332973, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 28, 221, 173, 182, 79, 4], OperandSize::Dword)
}

fn andnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 250], OperandSize::Qword)
}

fn andnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 2014600105, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 12, 141, 169, 91, 20, 120], OperandSize::Qword)
}

