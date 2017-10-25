use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 157, 242], OperandSize::Dword)
}

fn vfnmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1486712420, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 157, 12, 93, 100, 110, 157, 88], OperandSize::Dword)
}

fn vfnmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 157, 197], OperandSize::Qword)
}

fn vfnmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 157, 48], OperandSize::Qword)
}

fn vfnmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 153, 157, 234], OperandSize::Dword)
}

fn vfnmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 157, 4, 211], OperandSize::Dword)
}

fn vfnmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 157, 188, 157, 204], OperandSize::Qword)
}

fn vfnmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 133, 129, 157, 52, 142], OperandSize::Qword)
}

