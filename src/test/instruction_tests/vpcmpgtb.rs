use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 100, 194], OperandSize::Dword)
}

fn vpcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1981439976, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 100, 4, 157, 232, 95, 26, 118], OperandSize::Dword)
}

fn vpcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 100, 227], OperandSize::Qword)
}

fn vpcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 157158689, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 100, 4, 133, 33, 13, 94, 9], OperandSize::Qword)
}

fn vpcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 100, 210], OperandSize::Dword)
}

fn vpcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 100, 36, 89], OperandSize::Dword)
}

fn vpcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 100, 192], OperandSize::Qword)
}

fn vpcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 100, 44, 255], OperandSize::Qword)
}

fn vpcmpgtb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 10, 100, 214], OperandSize::Dword)
}

fn vpcmpgtb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 421109173, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 13, 100, 156, 241, 181, 157, 25, 25], OperandSize::Dword)
}

fn vpcmpgtb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 21, 3, 100, 232], OperandSize::Qword)
}

fn vpcmpgtb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 386473177, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 2, 100, 28, 189, 217, 28, 9, 23], OperandSize::Qword)
}

fn vpcmpgtb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 46, 100, 239], OperandSize::Dword)
}

fn vpcmpgtb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 47, 100, 44, 154], OperandSize::Dword)
}

fn vpcmpgtb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 35, 100, 212], OperandSize::Qword)
}

fn vpcmpgtb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RCX, 653191160, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 33, 100, 177, 248, 231, 238, 38], OperandSize::Qword)
}

fn vpcmpgtb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 75, 100, 220], OperandSize::Dword)
}

fn vpcmpgtb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 73, 100, 43], OperandSize::Dword)
}

fn vpcmpgtb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 61, 77, 100, 252], OperandSize::Qword)
}

fn vpcmpgtb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RCX, 376526144, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 29, 69, 100, 169, 64, 85, 113, 22], OperandSize::Qword)
}

