use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 185, 197], OperandSize::Dword)
}

fn vfmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 96016271, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 185, 132, 78, 143, 23, 185, 5], OperandSize::Dword)
}

fn vfmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 185, 236], OperandSize::Qword)
}

fn vfmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 185, 56], OperandSize::Qword)
}

fn vfmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 158, 185, 243], OperandSize::Dword)
}

fn vfmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 842847646, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 185, 140, 81, 158, 213, 60, 50], OperandSize::Dword)
}

fn vfmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 189, 186, 185, 255], OperandSize::Qword)
}

fn vfmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 781161371, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 129, 185, 180, 200, 155, 147, 143, 46], OperandSize::Qword)
}

