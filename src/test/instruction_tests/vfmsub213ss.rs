use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 171, 226], OperandSize::Dword)
}

fn vfmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 484419037, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 171, 182, 221, 165, 223, 28], OperandSize::Dword)
}

fn vfmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 171, 192], OperandSize::Qword)
}

fn vfmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 48631653, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 171, 52, 221, 101, 15, 230, 2], OperandSize::Qword)
}

fn vfmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 222, 171, 254], OperandSize::Dword)
}

fn vfmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 171, 33], OperandSize::Dword)
}

fn vfmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 29, 177, 171, 245], OperandSize::Qword)
}

fn vfmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RCX, 1242457955, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 45, 139, 171, 129, 99, 103, 14, 74], OperandSize::Qword)
}

