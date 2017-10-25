use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 121, 241], OperandSize::Dword)
}

fn vcvtss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 52, 87], OperandSize::Dword)
}

fn vcvtss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 126, 24, 121, 218], OperandSize::Qword)
}

fn vcvtss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1198462284, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 164, 115, 76, 21, 111, 71], OperandSize::Qword)
}

fn vcvtss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 254, 56, 121, 206], OperandSize::Qword)
}

fn vcvtss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1596097227, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 121, 36, 149, 203, 130, 34, 95], OperandSize::Qword)
}

