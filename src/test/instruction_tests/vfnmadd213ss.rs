use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 173, 204], OperandSize::Dword)
}

fn vfnmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 445513905, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 173, 167, 177, 0, 142, 26], OperandSize::Dword)
}

fn vfnmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 173, 221], OperandSize::Qword)
}

fn vfnmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1568727985, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 173, 140, 240, 177, 227, 128, 93], OperandSize::Qword)
}

fn vfnmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 252, 173, 231], OperandSize::Dword)
}

fn vfnmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 201008389, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 173, 167, 5, 37, 251, 11], OperandSize::Dword)
}

fn vfnmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 77, 250, 173, 224], OperandSize::Qword)
}

fn vfnmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1140415771, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 173, 60, 117, 27, 93, 249, 67], OperandSize::Qword)
}

