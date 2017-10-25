use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 169, 230], OperandSize::Dword)
}

fn vfmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 2046699898, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 169, 188, 90, 122, 41, 254, 121], OperandSize::Dword)
}

fn vfmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 169, 221], OperandSize::Qword)
}

fn vfmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1614957461, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 169, 156, 198, 149, 75, 66, 96], OperandSize::Qword)
}

fn vfmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 158, 169, 249], OperandSize::Dword)
}

fn vfmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1149738845, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 169, 190, 93, 159, 135, 68], OperandSize::Dword)
}

fn vfmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 237, 252, 169, 220], OperandSize::Qword)
}

fn vfmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1333218587, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 181, 138, 169, 60, 205, 27, 77, 119, 79], OperandSize::Qword)
}

