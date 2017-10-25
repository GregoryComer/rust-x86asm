use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 189, 246], OperandSize::Dword)
}

fn vfnmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 189, 28, 200], OperandSize::Dword)
}

fn vfnmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 189, 236], OperandSize::Qword)
}

fn vfnmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 1728231949, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 189, 170, 13, 186, 2, 103], OperandSize::Qword)
}

fn vfnmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 221, 189, 214], OperandSize::Dword)
}

fn vfnmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 189, 33], OperandSize::Dword)
}

fn vfnmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 197, 212, 189, 251], OperandSize::Qword)
}

fn vfnmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 229, 133, 189, 63], OperandSize::Qword)
}

