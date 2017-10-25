use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 153, 235], OperandSize::Dword)
}

fn vfmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 153, 7], OperandSize::Dword)
}

fn vfmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 250], OperandSize::Qword)
}

fn vfmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 52, 137], OperandSize::Qword)
}

fn vfmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 158, 153, 240], OperandSize::Dword)
}

fn vfmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 555047768, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 153, 172, 151, 88, 91, 21, 33], OperandSize::Dword)
}

fn vfmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 189, 180, 153, 248], OperandSize::Qword)
}

fn vfmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RDI, 1356139462, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 149, 138, 153, 175, 198, 11, 213, 80], OperandSize::Qword)
}

