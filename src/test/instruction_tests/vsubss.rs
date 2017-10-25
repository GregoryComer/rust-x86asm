use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsubss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 234], OperandSize::Dword)
}

fn vsubss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1633405754, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 92, 12, 117, 58, 203, 91, 97], OperandSize::Dword)
}

fn vsubss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 223], OperandSize::Qword)
}

fn vsubss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 945147339, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 92, 148, 207, 203, 205, 85, 56], OperandSize::Qword)
}

fn vsubss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 86, 159, 92, 235], OperandSize::Dword)
}

fn vsubss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1973069230, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 102, 137, 92, 166, 174, 165, 154, 117], OperandSize::Dword)
}

fn vsubss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 38, 242, 92, 224], OperandSize::Qword)
}

fn vsubss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1095565675, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 102, 131, 92, 148, 206, 107, 1, 77, 65], OperandSize::Qword)
}

