use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 169, 235], OperandSize::Dword)
}

fn vfmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 169, 40], OperandSize::Dword)
}

fn vfmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 169, 192], OperandSize::Qword)
}

fn vfmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 169, 44, 118], OperandSize::Qword)
}

fn vfmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 169, 213], OperandSize::Dword)
}

fn vfmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1765458401, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 169, 20, 85, 225, 193, 58, 105], OperandSize::Dword)
}

fn vfmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 45, 246, 169, 216], OperandSize::Qword)
}

fn vfmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RDI, 1415692141, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 109, 130, 169, 151, 109, 191, 97, 84], OperandSize::Qword)
}

