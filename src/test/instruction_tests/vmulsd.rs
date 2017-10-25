use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 89, 246], OperandSize::Dword)
}

fn vmulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1636214122, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 89, 36, 85, 106, 165, 134, 97], OperandSize::Dword)
}

fn vmulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 89, 233], OperandSize::Qword)
}

fn vmulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1414678220, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 89, 4, 253, 204, 70, 82, 84], OperandSize::Qword)
}

fn vmulsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 223, 155, 89, 229], OperandSize::Dword)
}

fn vmulsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 331773197, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 223, 139, 89, 180, 199, 13, 117, 198, 19], OperandSize::Dword)
}

fn vmulsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 255, 243, 89, 212], OperandSize::Qword)
}

fn vmulsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 2048753902, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 135, 143, 89, 28, 197, 238, 128, 29, 122], OperandSize::Qword)
}

