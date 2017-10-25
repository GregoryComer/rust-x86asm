use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 189, 209], OperandSize::Dword)
}

fn vfnmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 189, 36, 190], OperandSize::Dword)
}

fn vfnmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 189, 250], OperandSize::Qword)
}

fn vfnmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 189, 28, 185], OperandSize::Qword)
}

fn vfnmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 222, 189, 223], OperandSize::Dword)
}

fn vfnmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 189, 58], OperandSize::Dword)
}

fn vfnmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 13, 145, 189, 218], OperandSize::Qword)
}

fn vfnmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 789367606, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 85, 131, 189, 180, 246, 54, 203, 12, 47], OperandSize::Qword)
}

