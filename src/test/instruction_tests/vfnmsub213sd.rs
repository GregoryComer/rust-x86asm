use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 175, 192], OperandSize::Dword)
}

fn vfnmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 20, 251], OperandSize::Dword)
}

fn vfnmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 175, 212], OperandSize::Qword)
}

fn vfnmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 984354293, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 187, 245, 13, 172, 58], OperandSize::Qword)
}

fn vfnmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 250, 175, 213], OperandSize::Dword)
}

fn vfnmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 175, 35], OperandSize::Dword)
}

fn vfnmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 151, 175, 216], OperandSize::Qword)
}

fn vfnmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1141208525, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 133, 137, 175, 140, 195, 205, 117, 5, 68], OperandSize::Qword)
}

