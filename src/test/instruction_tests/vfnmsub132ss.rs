use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 159, 226], OperandSize::Dword)
}

fn vfnmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 299541214, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 159, 191, 222, 162, 218, 17], OperandSize::Dword)
}

fn vfnmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 159, 226], OperandSize::Qword)
}

fn vfnmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 159, 42], OperandSize::Qword)
}

fn vfnmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 255, 159, 221], OperandSize::Dword)
}

fn vfnmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 646089720, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 159, 128, 248, 139, 130, 38], OperandSize::Dword)
}

fn vfnmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 29, 154, 159, 203], OperandSize::Qword)
}

fn vfnmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 93, 139, 159, 20, 72], OperandSize::Qword)
}

