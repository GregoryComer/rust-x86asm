use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfpclassps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 242, 24], OperandSize::Dword)
}

fn vfpclassps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 28, 70, 77], OperandSize::Dword)
}

fn vfpclassps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 26, 102, 25, 9], OperandSize::Dword)
}

fn vfpclassps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 125, 10, 102, 212, 114], OperandSize::Qword)
}

fn vfpclassps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 10, 102, 12, 211, 76], OperandSize::Qword)
}

fn vfpclassps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 319531027, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 29, 102, 140, 155, 19, 168, 11, 19, 22], OperandSize::Qword)
}

fn vfpclassps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 47, 102, 227, 33], OperandSize::Dword)
}

fn vfpclassps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 47, 102, 44, 203, 47], OperandSize::Dword)
}

fn vfpclassps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(EDI, 1564937663, Some(OperandSize::Dword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 63, 102, 159, 191, 13, 71, 93, 115], OperandSize::Dword)
}

fn vfpclassps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 179, 125, 44, 102, 250, 58], OperandSize::Qword)
}

fn vfpclassps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1214738482, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 44, 102, 188, 191, 50, 112, 103, 72, 41], OperandSize::Qword)
}

fn vfpclassps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1592209975, Some(OperandSize::Dword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 59, 102, 148, 130, 55, 50, 231, 94, 20], OperandSize::Qword)
}

fn vfpclassps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 76, 102, 212, 21], OperandSize::Dword)
}

fn vfpclassps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1449065803, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 148, 190, 75, 253, 94, 86, 7], OperandSize::Dword)
}

fn vfpclassps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 2142568150, Some(OperandSize::Dword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 93, 102, 148, 113, 214, 254, 180, 127, 44], OperandSize::Dword)
}

fn vfpclassps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 125, 77, 102, 211, 17], OperandSize::Qword)
}

fn vfpclassps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 610190944, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 78, 102, 52, 93, 96, 198, 94, 36, 80], OperandSize::Qword)
}

fn vfpclassps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 90, 102, 58, 22], OperandSize::Qword)
}

