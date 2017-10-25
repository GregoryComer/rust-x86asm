use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 175, 218], OperandSize::Dword)
}

fn vfnmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 265291583, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 175, 182, 63, 7, 208, 15], OperandSize::Dword)
}

fn vfnmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 175, 232], OperandSize::Qword)
}

fn vfnmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 175, 1], OperandSize::Qword)
}

fn vfnmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 253, 175, 200], OperandSize::Dword)
}

fn vfnmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 240736537, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 175, 156, 203, 25, 89, 89, 14], OperandSize::Dword)
}

fn vfnmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 45, 222, 175, 227], OperandSize::Qword)
}

fn vfnmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 736500995, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 45, 138, 175, 140, 147, 3, 29, 230, 43], OperandSize::Qword)
}

