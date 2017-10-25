use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 217], OperandSize::Dword)
}

fn vfmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1244782998, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 155, 170, 150, 225, 49, 74], OperandSize::Dword)
}

fn vfmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 243], OperandSize::Qword)
}

fn vfmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 143455925, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 180, 177, 181, 246, 140, 8], OperandSize::Qword)
}

fn vfmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 159, 155, 208], OperandSize::Dword)
}

fn vfmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1635233808, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 155, 60, 133, 16, 176, 119, 97], OperandSize::Dword)
}

fn vfmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 117, 222, 155, 204], OperandSize::Qword)
}

fn vfmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 692079781, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 21, 142, 155, 60, 157, 165, 76, 64, 41], OperandSize::Qword)
}

