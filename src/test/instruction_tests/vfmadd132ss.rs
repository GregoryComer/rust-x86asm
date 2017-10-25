use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 153, 222], OperandSize::Dword)
}

fn vfmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 28, 179], OperandSize::Dword)
}

fn vfmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 202], OperandSize::Qword)
}

fn vfmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 153, 28, 135], OperandSize::Qword)
}

fn vfmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 159, 153, 249], OperandSize::Dword)
}

fn vfmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1629141564, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 153, 178, 60, 186, 26, 97], OperandSize::Dword)
}

fn vfmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 13, 147, 153, 231], OperandSize::Qword)
}

fn vfmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 131, 153, 35], OperandSize::Qword)
}

