use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 191, 218], OperandSize::Dword)
}

fn vfnmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1925704391, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 191, 164, 74, 199, 234, 199, 114], OperandSize::Dword)
}

fn vfnmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 191, 232], OperandSize::Qword)
}

fn vfnmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 191, 28, 241], OperandSize::Qword)
}

fn vfnmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 154, 191, 201], OperandSize::Dword)
}

fn vfnmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 191, 20, 71], OperandSize::Dword)
}

fn vfnmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 109, 187, 191, 236], OperandSize::Qword)
}

fn vfnmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 77, 142, 191, 48], OperandSize::Qword)
}

