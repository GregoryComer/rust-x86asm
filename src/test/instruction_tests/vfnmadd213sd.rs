use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 173, 247], OperandSize::Dword)
}

fn vfnmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 220384078, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 173, 44, 69, 78, 203, 34, 13], OperandSize::Dword)
}

fn vfnmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 173, 224], OperandSize::Qword)
}

fn vfnmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 1921997994, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 173, 166, 170, 92, 143, 114], OperandSize::Qword)
}

fn vfnmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 157, 173, 235], OperandSize::Dword)
}

fn vfnmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 959499421, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 173, 145, 157, 204, 48, 57], OperandSize::Dword)
}

fn vfnmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 229, 245, 173, 241], OperandSize::Qword)
}

fn vfnmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1957909134, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 173, 156, 201, 142, 82, 179, 116], OperandSize::Qword)
}

