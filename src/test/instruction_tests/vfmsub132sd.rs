use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 155, 228], OperandSize::Dword)
}

fn vfmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 974295133, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 155, 180, 222, 93, 144, 18, 58], OperandSize::Dword)
}

fn vfmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 155, 195], OperandSize::Qword)
}

fn vfmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1646080450, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 4, 205, 194, 49, 29, 98], OperandSize::Qword)
}

fn vfmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 223, 155, 204], OperandSize::Dword)
}

fn vfmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 197647273, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 155, 20, 189, 169, 219, 199, 11], OperandSize::Dword)
}

fn vfmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 229, 243, 155, 249], OperandSize::Qword)
}

fn vfmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 141, 133, 155, 44, 240], OperandSize::Qword)
}

